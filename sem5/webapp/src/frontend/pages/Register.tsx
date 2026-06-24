import LoginForm from "../components/LoginForm";

import { useNavigate } from "react-router";

/**
 * Register page for the application process of a new user.
 */
export default function Register() {
  const navigate = useNavigate();

  return (
    <>
      <div>
        <button onClick={() => navigate(-1)}>go back</button>
        <LoginForm form={"Register"} />
      </div>
    </>
  );
}
