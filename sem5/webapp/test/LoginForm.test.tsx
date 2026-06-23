import { describe, it, expect, vi } from "vitest";
import { render } from "@testing-library/react";
import { screen } from "@testing-library/dom";
import userEvent from "@testing-library/user-event";
import "@testing-library/jest-dom";
import LoginForm from "../src/components/LoginForm";
import axios from "axios";

vi.mock("axios");

describe("Testing LoginForm Component", () => {
  it("shouldn't display validation errors if empty fields are sent", async () => {
    const user = userEvent.setup();

    render(<LoginForm form="Register" />);

    const submitButton = screen.getByRole("button", { name: /Register/i });
    await user.click(submitButton);

    expect(await screen.findByText("Name is required")).toBeInTheDocument();
    expect(screen.getByText("Password is required")).toBeInTheDocument();
  });

  it("should display an error if passwords don't match", async () => {
    const user = userEvent.setup();

    render(<LoginForm form="Register" />);

    user.type(screen.getByPlaceholderText("Username"), "testuser");
    user.type(screen.getByPlaceholderText(/^Password$/i), "password123");
    user.type(screen.getByPlaceholderText("Confirm Password"), "different123");

    const submitButton = screen.getByRole("button", { name: /Register/i });
    user.click(submitButton);

    expect(
      await screen.findByText("Passwords do not match"),
    ).toBeInTheDocument();
  });

  it("should call Axios POST if the registration is valid", async () => {
    (axios.post as any).mockResolvedValue({ status: 201, data: {} });
    const user = userEvent.setup();

    render(<LoginForm form="Register" />);

    user.type(screen.getByPlaceholderText("Username"), "validUser");
    user.type(screen.getByPlaceholderText(/^Password$/i), "password123");
    user.type(screen.getByPlaceholderText("Confirm Password"), "password123");

    user.click(screen.getByRole("checkbox"));
    user.click(screen.getByRole("button", { name: /Register/i }));
  });
});
