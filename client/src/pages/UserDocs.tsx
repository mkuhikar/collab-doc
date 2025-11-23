import DocsList from "../components/DocsList";
import { useNavigate } from "react-router-dom";
import AppTopbar from "../components/AppTopBar";
import LeftSideBar from "../components/LeftSideBar";

export default function UserDocsPage() {
    const navigate = useNavigate();

    async function handleNewDoc() {
        const token = localStorage.getItem("authToken")?.replace(/^"|"$/g, "");

        try {
            const response = await fetch("http://104.197.202.203:3000/docs", {
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
                   <LeftSideBar/>

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
