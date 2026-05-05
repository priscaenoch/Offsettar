import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Offsetta",
  description: "Programmable mobile money ↔ blockchain financial rails",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
