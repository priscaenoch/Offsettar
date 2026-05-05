import { Router } from "express";

const router = Router();

// POST /v1/onramp/deposit
// Initiates a mobile money → USDC deposit
router.post("/deposit", (_req, res) => {
  res.status(501).json({ message: "Not implemented" });
});

// GET /v1/onramp/deposit/:id
router.get("/deposit/:id", (_req, res) => {
  res.status(501).json({ message: "Not implemented" });
});

export default router;
