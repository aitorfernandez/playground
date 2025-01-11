import express from "express";
import cors from "cors";

import { config } from "./handlers";

const app = express();
app.use(cors());
app.use(express.json());

config(app);

const PORT = 8080;
app.listen(PORT, () => {
	console.log(`Server is running on http://localhost:${PORT}`);
});
