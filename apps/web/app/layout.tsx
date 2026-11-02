import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Campus Circle — verified student exchange",
  description: "A privacy-first housing, marketplace, and rides community for verified Purdue students."
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return <html lang="en"><body>{children}</body></html>;
}

