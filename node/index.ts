import express, { Express, Request, Response } from "express";
import dotenv from "dotenv";
import OnnxRouter from "./src/routes/onnx.route";

dotenv.config();

export const _app: Express = express();
const port: number = process.env.PORT ? parseInt(process.env.PORT) : 3000;
_app.use(express.json()).use("/api", OnnxRouter);

_app.get("/test", (req: Request, res: Response) => {
  res.send("Express + TypeScript Server is running test");
});

if (process.env.NODE_ENV !== "test") {
  _app.listen(port, () => {
    console.log(`⚡️[server]: Server is running at http://localhost:${port}`);
  });
}

export const app = _app;
