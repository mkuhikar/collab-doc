import React from "react";

export default function AppTopbar() {
    return (
        <nav
            className="d-flex justify-content-between align-items-center px-4 py-3 bg-white shadow-sm"
            style={{ borderBottom: "1px solid #eee" }}
        >
            <h4 className="fw-bold m-0">Notes</h4>

            <div className="d-flex align-items-center gap-3">

                {/* Search Box */}
                <div className="position-relative search-box">
                    <i className="bi bi-search search-icon"></i>
                    <input
                        type="text"
                        className="form-control rounded-pill ps-5"
                        placeholder="Search..."
                    />
                </div>

                {/* Profile Avatar */}
                <img
                    src="https://i.pravatar.cc/40"
                    alt="User avatar"
                    className="rounded-circle"
                    style={{ width: "40px", height: "40px" }}
                />
            </div>
        </nav>
    );
}
