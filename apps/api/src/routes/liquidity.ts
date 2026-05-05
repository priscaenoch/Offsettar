import { Router } from "express";

const router = Router();

// GET /v1/liquidity/pools
router.get("/pools", (_req, res) => {
  res.status(501).json({ message: "Not implemented" });
});

// GET /v1/liquidity/pools/:id
router.get("/pools/:id", (_req, res) => {
  res.status(501).json({ message: "Not implemented" });
});

export default router;
