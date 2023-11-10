import express from "express";
import multer from "multer";
import { OnnxController } from "../controllers/onnx.controller";
import { exec } from "child_process";

const onnxController = new OnnxController();
const router = express.Router();

const upload = multer({ dest: "uploads/" });

router.post("/", upload.single("file"), onnxController.uploadOnnx);

router.post("/compile", (req, res) => {
  const command = `pwd && cd stark_prover`;

  exec(
    command,
    { shell: "/bin/bash", cwd: "/app/stark_prover" },
    (error, stdout, stderr) => {
      if (error) {
        console.error(`Error executing command: ${error}`);
        res.status(500).send("Error compiling");
        return;
      }

      if (stderr) {
        console.error(`Command error output: ${stderr}`);
        res.status(500).send("Error compiling");
        return;
      }

      // If no errors, you can process the stdout if needed
      console.log(`Command output: ${stdout}`);

      // Send a success response
      res.status(200).send("Compilation successful");
    }
  );
});

export default router;
