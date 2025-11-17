import React from "react";
import { Form, Button, Card } from "react-bootstrap";

export default function ProfilePage() {
  return (
    <div
      className="container-fluid"
      style={{
        background: "#f8f9fa",
        minHeight: "100vh",
        padding: "40px",
      }}
    >
      <div className="row justify-content-center">
        <div className="col-lg-6 col-md-8 col-sm-12">

          <Card
            className="p-4 shadow-sm"
            style={{
              borderRadius: "20px",
              background: "white",
            }}
          >
            {/* Top section */}
            <div className="d-flex align-items-center mb-4">
              <img
                src="/profile-avatar.png"
                alt="avatar"
                width="70"
                height="70"
                style={{
                  borderRadius: "50%",
                  objectFit: "cover",
                  marginRight: "20px",
                }}
              />
              <div>
                <h3 className="mb-0 fw-semibold" style={{ letterSpacing: "-0.5px" }}>
                  Your Profile
                </h3>
                <small className="text-muted">Manage your information</small>
              </div>
            </div>

            {/* FORM */}
            <Form>

              <Form.Group className="mb-3">
                <Form.Label className="fw-medium">Full Name</Form.Label>
                <Form.Control
                  type="text"
                  defaultValue="Ramesh Shinde"
                  style={{
                    borderRadius: "12px",
                    padding: "12px",
                    border: "1px solid #e6e6e6",
                  }}
                />
              </Form.Group>

              <Form.Group className="mb-3">
                <Form.Label className="fw-medium">Email</Form.Label>
                <Form.Control
                  type="email"
                  defaultValue="ramesh@example.com"
                  disabled
                  style={{
                    borderRadius: "12px",
                    padding: "12px",
                    border: "1px solid #e6e6e6",
                    background: "#fafafa",
                  }}
                />
                <small className="text-muted">Email cannot be changed</small>
              </Form.Group>

              {/* Tags / Interests section */}
              <Form.Group className="mb-4">
                <Form.Label className="fw-medium">Tags</Form.Label>
                <div className="d-flex flex-wrap gap-2">
                  <span className="badge rounded-pill px-3 py-2" style={{ background: "#ffe9a9" }}>
                    Work
                  </span>
                  <span className="badge rounded-pill px-3 py-2" style={{ background: "#ffd6d6" }}>
                    Ideas
                  </span>
                  <span className="badge rounded-pill px-3 py-2" style={{ background: "#d2f6db" }}>
                    Personal
                  </span>
                </div>
              </Form.Group>

              <Button
                className="w-100 py-2 fw-semibold"
                style={{
                  borderRadius: "12px",
                  background: "#000",
                }}
              >
                Save Changes
              </Button>
            </Form>

            {/* Divider */}
            <hr className="my-4" />

            {/* Settings section */}
            <div>
              <h5 className="fw-semibold mb-3">Account Settings</h5>

              <div className="list-group">

                <div className="list-group-item border-0 px-0 d-flex justify-content-between align-items-center">
                  <span className="fw-medium">Change Password</span>
                  <i className="bi bi-chevron-right"></i>
                </div>

                <div className="list-group-item border-0 px-0 d-flex justify-content-between align-items-center">
                  <span className="fw-medium">Notifications</span>
                  <i className="bi bi-chevron-right"></i>
                </div>

                <div className="list-group-item border-0 px-0 d-flex justify-content-between align-items-center">
                  <span className="fw-medium">Theme</span>
                  <i className="bi bi-chevron-right"></i>
                </div>

                <div className="list-group-item border-0 px-0 d-flex justify-content-between align-items-center text-danger">
                  <span className="fw-medium">Logout</span>
                  <i className="bi bi-box-arrow-right"></i>
                </div>

              </div>
            </div>

          </Card>
        </div>
      </div>
    </div>
  );
}
