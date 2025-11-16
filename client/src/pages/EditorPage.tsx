import { useEffect, useState, useRef } from "react";
import { useParams } from "react-router-dom";
import { Button, Modal, Form, Spinner, Alert } from "react-bootstrap";
import { v4 as uuidv4 } from "uuid";
import AppNavbar from "../components/Navbar";

export default function EditorPage() {
  const skipNextRender = useRef(false);
  const isUserTypingRef = useRef(false);

  const { id } = useParams();
  const [content, setContent] = useState("");
  const [loading, setLoading] = useState(true);
  const [role, setRole] = useState("Editor");
  const wsRef = useRef<WebSocket | null>(null);
  const clientId = useRef<string>(uuidv4());
  const editorRef = useRef<HTMLDivElement | null>(null);
  const clientVersionRef = useRef<number>(0);

  const [showShareModal, setShowShareModal] = useState(false);
  const [shareEmail, setShareEmail] = useState("");
  const [shareRole, setShareRole] = useState<"Reader" | "Editor">("Reader");

  // Fetch document + role
  useEffect(() => {
    async function fetchDoc() {
      if (!id) return;
      console.log("📡 Fetching document:", id);
      try {
        const token = localStorage.getItem("authToken")?.replace(/^"|"$/g, "");
        const res = await fetch(`http://localhost:3000/docs/${id}`, {
          headers: { Authorization: `Bearer ${token}` },
        });
        console.log("🧾 Response status:", res.status);
        if (!res.ok) throw new Error("Failed to fetch document");
        const data = await res.json();
        console.log("📄 Document data:", data);
        setContent(data.content || "");
        setRole(data.role || "Reader");
      } catch (err) {
        console.error("❌ Error loading document:", err);
        alert("Failed to load document content.");
      } finally {
        setLoading(false);
      }
    }
    fetchDoc();
  }, [id]);
  useEffect(() => {
  if (!editorRef.current) return;

  // ⬅️ Skip DOM overwrite if change came from user typing
  if (skipNextRender.current) {
    console.log("⏭️ Skipping DOM update after local input");
    skipNextRender.current = false;
    return;
  }

  console.log("🔧 Setting editor text. Role:", role);
  editorRef.current.textContent = content;

  editorRef.current.setAttribute(
    "contenteditable",
    role !== "Reader" ? "true" : "false"
  );
}, [content, role]);


 

// WebSocket setup
useEffect(() => {
  if (!id || loading || role === "Reader") {
    console.log("🚫 Skipping WS connection - Role:", role, "Loading:", loading);
    return;
  }

  const wsUrl = `ws://localhost:3000/ws/docs/${id}`;
  console.log("🌐 Connecting to WebSocket:", wsUrl, "as role:", role);

  const socket = new WebSocket(wsUrl);
  wsRef.current = socket;

  socket.onopen = () => {
    console.log("✅ WebSocket connected successfully");
    socket.send(JSON.stringify({ type: "join", client_id: clientId.current }));
  };

  socket.onerror = (err) => {
    console.error("❌ WebSocket error:", err);
  };

  socket.onclose = (event) => {
    console.warn("⚠️ WebSocket closed:", event.reason || event);
  };

  socket.onmessage = (event) => {
    console.log("📨 Incoming WS message:", event.data);

    try {
      const msg = JSON.parse(event.data);
      console.log("📦 Parsed WS message:", msg);

      if (msg.client_id && msg.client_id === clientId.current) {
        console.log("↩️ Ignoring own message");
        if (msg.version !== undefined) {
        clientVersionRef.current = msg.version;
      }
      return;
      }
      if (msg.type === "force_refresh") {
      console.warn("🔄 FORCE REFRESH received — resetting local content!");
      console.log("🔢 Server version:", msg.version);

      // Update content and version
      clientVersionRef.current = msg.version;
      setContent(msg.content);

      // Update editor safely
      if (editorRef.current) {
        editorRef.current.textContent = msg.content;
      }

      return;
    }

      if (msg.type === "doc_update") {
  console.log("📝 Remote update received (v", msg.version, ")");

  // Do NOT overwrite if the local user is typing right now
  if (isUserTypingRef.current) {
    console.log("⏸️ Skipping remote DOM update because user is typing");
    // Still update version so client stays in sync
    clientVersionRef.current = msg.version;
    return;
  }

  console.log("🔧 Applying remote update to DOM");

  clientVersionRef.current = msg.version;
  setContent(msg.content);

  if (editorRef.current && editorRef.current.textContent !== msg.content) {
    editorRef.current.textContent = msg.content;
  }

  return;
}
    
  } catch (err) {
    console.error("❌ Failed to parse WS message:", err);
  }
  };

  return () => {
    console.log("🧹 Cleaning up WebSocket connection");
    socket.close();
  };
}, [id, loading, role]);

  // Diff function
  function findDiffStart(a: string, b: string): number {
    let i = 0;
    while (i < a.length && i < b.length && a[i] === b[i]) i++;
    return i;
  }

  // Input handler
  const handleInput = () => {
    
  if (!editorRef.current || role === "Reader") {
    console.log("🚫 Ignored input — role:", role);
    return;
  }


  skipNextRender.current = true; // ⬅️ prevent DOM overwrite on next render
isUserTypingRef.current = true;

setTimeout(() => {
  isUserTypingRef.current = false;
}, 200); // you can tune (150–250ms is ideal)
  const newText = editorRef.current.textContent || "";
  const oldText = content;

  console.log("✏️ Input detected — old length:", oldText.length, "new length:", newText.length);

  let op = null;
  if (newText.length > oldText.length) {
    const pos = findDiffStart(oldText, newText);
    const inserted = newText.slice(pos, pos + (newText.length - oldText.length));
    op = { type: "Insert", pos, text: inserted };
    console.log("➕ Insert operation:", op);
  } else if (newText.length < oldText.length) {
    const pos = findDiffStart(newText, oldText);
    const len = oldText.length - newText.length;
    op = { type: "Delete", pos, len };
    console.log("➖ Delete operation:", op);
  }

  if (op) {
    clientVersionRef.current += 1;
    const message = {
      type: "doc_update",
      client_id: clientId.current,
      client_version: clientVersionRef.current,
      op,
      content: newText,
    };
    console.log("📤 Sending WS message:", message);
    wsRef.current?.send(JSON.stringify(message));
  }

  setContent(newText); // triggers effect, but skipNextRender stops overwrite
};

  // Apply contentEditable
  useEffect(() => {
    if (!editorRef.current) return;
    console.log("🔧 Setting editor text. Role:", role);
    editorRef.current.textContent = content;
    editorRef.current.setAttribute("contenteditable", role !== "Reader" ? "true" : "false");
  }, [content, role]);

  // Prevent input for readers
  useEffect(() => {
    if (!editorRef.current) return;
    const el = editorRef.current;

    if (role === "Reader") {
      console.log("🔒 Setting up input block for Reader");
      const preventEdit = (e: Event) => {
        e.preventDefault();
        console.log("🚫 Blocked edit attempt");
      };
      el.addEventListener("keydown", preventEdit);
      el.addEventListener("paste", preventEdit);
      el.addEventListener("input", preventEdit);

      return () => {
        console.log("🧹 Removing Reader event blocks");
        el.removeEventListener("keydown", preventEdit);
        el.removeEventListener("paste", preventEdit);
        el.removeEventListener("input", preventEdit);
      };
    }
  }, [role]);

  if (loading) {
    return (
      <div className="d-flex align-items-center justify-content-center vh-100">
        <Spinner animation="border" role="status" />
      </div>
    );
  }
  async function shareDocument() {
  if (!shareEmail) {
    alert("Please enter an email.");
    return;
  }

  try {
    console.log("📤 Sending share request...");

    const token = localStorage.getItem("authToken")?.replace(/^"|"$/g, "");

    const res = await fetch(`http://localhost:3000/docs/${id}/share`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify({
        email: shareEmail,
        role: shareRole,
      }),
    });

    console.log("📥 Share API response status:", res.status);

    if (!res.ok) {
      const text = await res.text();
      console.error("❌ Share failed:", text);
      alert("Failed to share: " + text);
      return;
    }

    alert("Document shared successfully!");
    setShowShareModal(false);
    setShareEmail("");

  } catch (err) {
    console.error("💥 Network/server error:", err);
    alert("Error sharing document.");
  }
}


  return (
    <>
      <AppNavbar />
      <div className="container-fluid bg-light vh-100 p-0 d-flex flex-column">
        <div className="bg-white border-bottom shadow-sm py-2 px-3 d-flex align-items-center justify-content-between">
  <h5 className="mb-0 fw-semibold">
    {role === "Reader" ? "Viewing Document" : "Editing Document"}
  </h5>

  {(role === "Editor"||role ==="Owner") && (
    <Button variant="primary" className="btn btn-info" onClick={() => setShowShareModal(true)}>
      Share
    </Button>
  )}
</div>


        {role === "Reader" && (
          <Alert variant="warning" className="text-center rounded-0 m-0">
            You can only view this document.
          </Alert>
        )}

        <div
          ref={editorRef}
          className={`flex-grow-1 bg-white p-4 overflow-auto ${
            role === "Reader" ? "text-muted" : ""
          }`}
          contentEditable={role !== "Reader"}
          suppressContentEditableWarning
          onInput={handleInput}
          style={{
            minHeight: "80vh",
            outline: "none",
            border: "none",
            fontSize: "1.1rem",
            whiteSpace: "pre-wrap",
            wordBreak: "break-word",
          }}
        />
      </div>
      <Modal show={showShareModal} onHide={() => setShowShareModal(false)}>
  <Modal.Header closeButton>
    <Modal.Title>Share Document</Modal.Title>
  </Modal.Header>

  <Modal.Body>
    <Form>
      <Form.Group className="mb-3">
        <Form.Label>Email</Form.Label>
        <Form.Control
          type="email"
          placeholder="Enter email"
          value={shareEmail}
          onChange={(e) => setShareEmail(e.target.value)}
        />
      </Form.Group>

      <Form.Group className="mb-3">
        <Form.Label>Role</Form.Label>
        <Form.Select
          value={shareRole}
          onChange={(e) => setShareRole(e.target.value as "Reader" | "Editor")}
        >
          <option value="Reader">Reader</option>
          <option value="Editor">Editor</option>
        </Form.Select>
      </Form.Group>
    </Form>
  </Modal.Body>

  <Modal.Footer>
    <Button variant="secondary" onClick={() => setShowShareModal(false)}>
      Cancel
    </Button>
    <Button variant="primary" onClick={shareDocument}>
      Share
    </Button>
  </Modal.Footer>
</Modal>

    </>
  );
}
