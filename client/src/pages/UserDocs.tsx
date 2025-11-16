import DocsList from "../components/DocsList";
import { useNavigate } from "react-router-dom";
import AppNavbar from "../components/Navbar";
import AppTopbar from "../components/AppTopBar";

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
                throw new Error(`Failed: ${text}`);
            }

            const newDoc = await response.json();
            navigate(`/editor/${newDoc.id}`);
        } catch (error) {
            console.error(error);
            alert("❌ Could not create a document.");
        }
    }

    return (
        <>
            <AppTopbar />

            <div className="container-fluid bg-light" style={{ height: "100vh" }}>
                <div className="row h-100">

                    {/* ---------- LEFT SIDEBAR ---------- */}
                    <div className="col-12 col-md-3 col-lg-2 bg-white shadow-sm p-4 sidebar-custom">


                        <ul className="list-unstyled sidebar-menu">
                            <li className="active mb-3">
                                <i className="bi bi-journal-text me-2"></i> All Notes
                            </li>
                            <li className="mb-3">
                                <i className="bi bi-star me-2"></i> Favorites
                            </li>
                            <li className="mb-3">
                                <i className="bi bi-tags me-2"></i> Tags
                            </li>
                            <li className="mb-3">
                                <i className="bi bi-people me-2"></i> Shared with me
                            </li>
                            <li className="mb-3">
                                <i className="bi bi-clock-history me-2"></i> Recent Notes
                            </li>
                            <li className="mb-3">
                                <i className="bi bi-trash me-2"></i> Trash
                            </li>
                        </ul>

                        <hr />

                        <p className="fw-semibold small text-muted">Tags</p>
                        <div className="d-flex flex-column gap-1">
                            <span><span className="tag-dot bg-warning"></span> Work</span>
                            <span><span className="tag-dot bg-primary"></span> Ideas</span>
                            <span><span className="tag-dot bg-success"></span> Personal</span>
                            <span><span className="tag-dot bg-info"></span> Projects</span>
                        </div>

                        <button
                            className="btn btn-outline-secondary mt-4 w-100"
                            onClick={handleNewDoc}
                        >
                            + New Note
                        </button>
                    </div>

                    {/* ---------- MAIN CONTENT ---------- */}
                    <div className="col p-5">

                        {/* Top Right Search */}
                        <div className="d-flex justify-content-between align-items-center mb-4">
                            <div>
                                <h2 className="fw-bold">Welcome back, Ramesh</h2>
                                <p className="text-muted">
                                    What would you like to write today?
                                </p>
                            </div>

                           
                        </div>

                        {/* Notes Grid */}
                        <div className="row g-4">
                            <DocsList />
                        </div>

                        {/* Floating Add Button */}
                        <button
                            className="btn btn-warning rounded-circle shadow-lg floating-add-btn"
                            onClick={handleNewDoc}
                        >
                            +
                        </button>
                    </div>
                </div>
            </div>
        </>
    );
}
