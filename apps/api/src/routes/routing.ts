import { Router } from "express";

const router = Router();

// POST /v1/routing/quote
// Returns best route + fee estimate for a given corridor
router.post("/quote", (_req, res) => {
  res.status(501).json({ message: "Not implemented" });
});

export default router;
