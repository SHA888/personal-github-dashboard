import React, { useState } from "react";
import { Box, Button, TextInput, Heading, Text } from "@primer/react";
import { useAuth } from "../hooks/useAuth";

const LoginPage: React.FC = () => {
  const [pat, setPat] = useState("");
  const { loginWithGitHub, loginWithPat } = useAuth();

  return (
    <Box sx={{ maxWidth: 400, mx: "auto", py: 4 }}>
      <Heading sx={{ mb: 3 }}>Login</Heading>
      <Button onClick={loginWithGitHub} sx={{ width: "100%", mb: 3 }}>
        Login with GitHub
      </Button>
      <Text sx={{ mb: 1, textAlign: "center" }}>— OR —</Text>
      <TextInput
        type="text"
        placeholder="Personal Access Token"
        value={pat}
        onChange={(e) => setPat(e.target.value)}
        sx={{ width: "100%", mb: 2 }}
      />
      <Button onClick={() => loginWithPat(pat)} sx={{ width: "100%" }}>
        Login with PAT
      </Button>
    </Box>
  );
};

export default LoginPage;
