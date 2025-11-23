// src/components/DocsList.tsx
import { useEffect, useState } from "react";

interface Doc {
  id: string;
  title: string;
  content: string;
  updated_at: string;
}

export default function DocsList() {
  const [docs, setDocs] = useState<Doc[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  useEffect(() => {
    async function fetchDocs() {
      console.log("local storage", localStorage.getItem("authToken"));
      const token = localStorage.getItem("authToken")?.replace(/^"|"$/g, "").trim();
      if (!token) {
        setError("No token found — please log in again.");
        setLoading(false);
        return;
      }
      try {
        console.log("docs page", token);
        const res = await fetch("http://104.197.202.203:3000/users/docs", {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        });

        if (!res.ok) throw new Error("Failed to load documents");
        const data = await res.json();
        setDocs(data);
      } catch (err) {
        setError("❌ Could not fetch documents.");
      } finally {
        setLoading(false);
      }
    }

    fetchDocs();
  }, []);

  if (loading) return <p>Loading documents...</p>;
  if (error) return <div className="alert alert-danger">{error}</div>;

  if (docs.length === 0)
    return (
      <div className="text-center mt-5">
        <h5>No documents yet.</h5>
        <p>Click below to create your first one.</p>
      </div>
    );

  return (
    <div className="container mt-4">
      <div className="row g-4">
        {docs.map((doc) => (
          <div key={doc.id} className="col-12 col-sm-6 col-md-4 col-lg-3">
            <a
              href={`http://104.197.202.203:3000/editor/${doc.id}`}
              className="text-decoration-none text-dark"
            >
              <div className="card h-100 shadow border-0 rounded-3 hover-shadow">
                <div className="card-body">
                  <h6 className="card-title fw-semibold mb-2 text-truncate">
                    {doc.title || "Untitled Document"}
                  </h6>
                  <p className="card-text text-muted small mb-3" style={{ minHeight: "3em" }}>
                    {doc.content ? doc.content.slice(0, 80) + "..." : "No content yet."}
                  </p>
                </div>
                <div className="card-footer bg-transparent border-0 text-end small text-muted">
                  {new Date(doc.updated_at).toLocaleDateString()}
                </div>
              </div>
            </a>
          </div>
        ))}
      </div>
    </div>
  );
}
