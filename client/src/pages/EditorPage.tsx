// src/pages/EditorPage.tsx
import { useEffect, useState, useRef } from "react";
import { useParams } from "react-router-dom";
import { Button } from "react-bootstrap";
import { v4 as uuidv4 } from "uuid";
import AppNavbar from "../components/Navbar";

export default function EditorPage() {
  const { id } = useParams(); // doc_id from URL
  const [content, setContent] = useState("");
  const wsRef = useRef<WebSocket | null>(null);
  const clientId = useRef<string>(uuidv4());
  const editorRef = useRef<HTMLDivElement | null>(null);

  // --- Connect WebSocket ---
  useEffect(() => {
    if (!id) return;

    const ws = new WebSocket(`ws://localhost:3000/ws/docs/${id}`);
    wsRef.current = ws;

    ws.onopen = () => {
      console.log("✅ Connected to WebSocket");
      const joinMsg = { type: "join", client_id: clientId.current };
      ws.send(JSON.stringify(joinMsg));
    };

    ws.onmessage = (event) => {
      const msg = JSON.parse(event.data);
      console.log("📩 Received from backend:", msg);

      // Ignore echo messages
      if (msg.client_id && msg.client_id === clientId.current) return;

      if (msg.type === "doc_update") {
        setContent(msg.content);

        // ✅ Update editor text safely without triggering cursor jump
        if (
          editorRef.current &&
          editorRef.current.textContent !== msg.content
        ) {
          const selection = window.getSelection();
          const range = selection?.getRangeAt(0);
          const cursorPos = range
            ? range.startOffset
            : editorRef.current.textContent?.length || 0;

          editorRef.current.textContent = msg.content;

          // Restore cursor safely
          const newRange = document.createRange();
          const textNode = editorRef.current.firstChild;
          if (textNode) {
            const pos = Math.min(cursorPos, textNode.textContent?.length || 0);
            newRange.setStart(textNode, pos);
            newRange.collapse(true);
            selection?.removeAllRanges();
            selection?.addRange(newRange);
          }
        }
      }
    };

    ws.onclose = () => console.log("❌ Disconnected from WebSocket");
    return () => ws.close();
  }, [id]);

  const clientVersionRef = useRef<number>(0); // track local version

function findDiffStart(a: string, b: string): number {
  let i = 0;
  while (i < a.length && i < b.length && a[i] === b[i]) i++;
  return i;
}

  const handleInput = () => {
  if (!editorRef.current) return;

  const newText = editorRef.current.textContent || "";

  // Compute a diff to detect what changed
  const oldText = content;
  let op = null;

  if (newText.length > oldText.length) {
    // insertion
    const pos = findDiffStart(oldText, newText);
    const inserted = newText.slice(pos, pos + (newText.length - oldText.length));
    op = { type: "Insert", pos, text: inserted };
  } else if (newText.length < oldText.length) {
    // deletion
    const pos = findDiffStart(newText, oldText);
    const len = oldText.length - newText.length;
    op = { type: "Delete", pos, len };
  }

  // only send if there was an actual change
  if (op) {
    clientVersionRef.current += 1;

    const message = {
      client_id: clientId.current,
      client_version: clientVersionRef.current,
      op,
    };

    console.log("✉️ Sending operation to backend:", message);
    wsRef.current?.send(JSON.stringify(message));
  }

  setContent(newText);
};
  // --- Save to backend ---
  const handleSave = async () => {
    const token = localStorage.getItem("authToken")?.replace(/^"|"$/g, "");
    console.log("💾 Saving content via HTTP PUT:", content);

    await fetch(`http://localhost:3000/docs`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify({ title: "Untitled", content }),
    });

    alert("💾 Saved!");
  };

  // --- Initialize editor text on mount ---
  useEffect(() => {
    if (editorRef.current && content) {
      editorRef.current.textContent = content;
    }
  }, [content]);

  // --- UI ---
  return (
    <>
    <AppNavbar/>
    <div className="container-fluid bg-light vh-100 p-0 d-flex flex-column">
      {/* Toolbar */}
      <div className="bg-white border-bottom shadow-sm py-2 px-3 d-flex align-items-center justify-content-between">
        <h5 className="mb-0 fw-semibold">Editing Document</h5>
        <Button variant="primary" className="fw-semibold" onClick={handleSave}>
          <i className="bi bi-save me-2"></i> Save
        </Button>
      </div>

      {/* Editor */}
      <div
        ref={editorRef}
        className="flex-grow-1 bg-white p-4 overflow-auto"
        contentEditable
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
    </>
  );
}
