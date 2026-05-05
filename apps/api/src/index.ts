import express from "express";
import cors from "cors";
import helmet from "helmet";
import "dotenv/config";

import onrampRouter from "./routes/onramp";
import offrampRouter from "./routes/offramp";
import routingRouter from "./routes/routing";
import liquidityRouter from "./routes/liquidity";

const app = express();

app.use(helmet());
app.use(cors());
app.use(express.json());

app.get("/health", (_req, res) => res.json({ status: "ok" }));

app.use("/v1/onramp", onrampRouter);
app.use("/v1/offramp", offrampRouter);
app.use("/v1/routing", routingRouter);
app.use("/v1/liquidity", liquidityRouter);

const PORT = process.env.PORT ?? 4000;
app.listen(PORT, () => console.log(`Offsetta API running on :${PORT}`));
