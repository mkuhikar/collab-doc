// src/components/Navbar.tsx
import { Navbar, Nav, Button, Container } from "react-bootstrap";
import { useNavigate } from "react-router-dom";

export default function AppNavbar() {
  const navigate = useNavigate();

  const handleLogout = () => {
    localStorage.removeItem("authToken");
    navigate("/login");
  };

  return (
    <Navbar bg="light" expand="sm" className="border-bottom shadow-sm">
      <Container fluid>
        <Nav className="me-auto d-flex align-items-center">
          <Button
            variant="outline-primary"
            className="me-2 fw-semibold"
            onClick={() => navigate("/user/docs")}
          >
            🏠 Home
          </Button>

          <Button
            variant="outline-danger"
            className="fw-semibold"
            onClick={handleLogout}
          >
            🚪 Logout
          </Button>
        </Nav>
      </Container>
    </Navbar>
  );
}
