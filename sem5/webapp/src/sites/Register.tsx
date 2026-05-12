import LoginForm from "../components/LoginForm";

import { useNavigate } from "react-router";

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
