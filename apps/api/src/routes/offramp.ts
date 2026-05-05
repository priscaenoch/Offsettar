import { Router } from "express";

const router = Router();

// POST /v1/offramp/withdraw
// Initiates a USDC → mobile money withdrawal
router.post("/withdraw", (_req, res) => {
  res.status(501).json({ message: "Not implemented" });
});

// GET /v1/offramp/withdraw/:id
router.get("/withdraw/:id", (_req, res) => {
  res.status(501).json({ message: "Not implemented" });
});

export default router;
