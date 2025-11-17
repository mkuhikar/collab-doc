import React from "react";
import { Dropdown } from "react-bootstrap";
import { useNavigate } from "react-router-dom";


type Props = {
  title?: string;
  actions?: React.ReactNode; // put Share button here from EditorPage
};



export default function AppTopBar({ title = "Notes", actions }: Props) {

   const navigate = useNavigate();

  const handleLogout = () => {
    localStorage.removeItem("authToken");
    navigate("/login");
  };
  return (
    <header className="app-topbar bg-white border-bottom shadow-sm">
      <div className="container-fluid">
        <div className="d-flex align-items-center justify-content-between py-2">
          {/* Left: brand/title */}
          <div className="d-flex align-items-center gap-3">
            <div className="fs-4 fw-bold">{title}</div>
          </div>

          {/* Right: actions injected by page (Share), then search, then avatar/dropdown */}
          <div className="d-flex align-items-center gap-3">
            {/* Actions area (Share button inserted by parent) */}
            {actions && <div className="d-flex align-items-center">{actions}</div>}

            {/* Search input */}
            <div>
              <div className="input-group rounded-pill" style={{ minWidth: 220,
                borderRadius: 50,
                border: "1px solid #ddd",   // ← visible border
                overflow: "hidden",         // ← keeps pill shape clean
                background: "#fff" }}>
                <span className="input-group-text bg-white border-0" style={{ borderRadius: "50px 0 0 50px" }}>
                  <i className="bi bi-search"></i>
                </span>
                <input
                  className="form-control border-0"
                  placeholder="Search..."
                  aria-label="Search"
                  style={{ borderRadius: "0 50px 50px 0", boxShadow: "none" }}
                />
              </div>
            </div>

            {/* Avatar + dropdown */}
            <Dropdown align="end">
              <Dropdown.Toggle variant="link" className="p-0 border-0">
                <img
                    src="https://i.pravatar.cc/40"
                    alt="User avatar"
                    className="rounded-circle"
                    style={{ width: "40px", height: "40px" }}
                />
              </Dropdown.Toggle>

              <Dropdown.Menu>
                <Dropdown.Item>Profile</Dropdown.Item>
                <Dropdown.Item>Settings</Dropdown.Item>
                <Dropdown.Divider />
                <Dropdown.Item onClick={handleLogout}>Logout</Dropdown.Item>
              </Dropdown.Menu>
            </Dropdown>
          </div>
        </div>
      </div>
    </header>
  );
}
