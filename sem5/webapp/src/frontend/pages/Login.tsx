import LoginForm from "../components/LoginForm";

import { useNavigate } from "react-router";

/**
 * Login page for the process of logging into an already
 * existing account.
 */
export default function Login() {
  const navigate = useNavigate();

  return (
    <>
      <div>
        <button onClick={() => navigate(-1)}>go back</button>
        <LoginForm form={"Login"} />
      </div>
    </>
  );
}
