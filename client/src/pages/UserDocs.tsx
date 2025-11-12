// src/pages/UserDocsPage.tsx
import DocsList from "../components/DocsList";
import { useNavigate } from "react-router-dom";
import AppNavbar from "../components/Navbar";

export default function UserDocsPage() {
    const navigate = useNavigate();

    async function handleNewDoc() {
    const token = localStorage.getItem("authToken")?.replace(/^"|"$/g, "");

    try {
        const response = await fetch("http://localhost:3000/docs", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify({
            title: "New Document",
            content: "",
        }),
        });

        if (!response.ok) {
        const text = await response.text();
        throw new Error(`Failed to create document: ${text}`);
        }

        // Assuming your backend returns the created document (with id, title, etc.)
        const newDoc = await response.json();

        // Redirect to editor page with the new document ID
        navigate(`/editor/${newDoc.id}`);
    } catch (error) {
        console.error("Error creating new document:", error);
        alert("❌ Failed to create new document. Check console for details.");
    }
    }
    return (
        <>
        <AppNavbar/>
        <div className="container py-5">
        <div className="text-center mb-4">
            <h2 className="fw-bold mb-3">📄 Your Documents</h2>
            <p className="text-muted">View, edit, and collaborate on your existing documents</p>
            <button className="btn btn-info fw-semibold px-4" onClick={handleNewDoc}>
            + Create New Document
            </button>
        </div>

        <div className="mx-auto" style={{ maxWidth: "700px" }}>
            <DocsList />
        </div>
        </div>
        </>
    );
}
