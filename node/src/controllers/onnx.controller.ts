import { Request, Response } from "express";
import multer from "multer";

export class OnnxController {
  constructor() {}

  uploadOnnx(req: Request, res: Response) {
    try {
      const file = req.body.file;
    } catch (e) {}
  }
}
