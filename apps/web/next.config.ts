import type { NextConfig } from "next";

// Static export: the app is fully client-rendered and talks to the Rust API
// over NEXT_PUBLIC_API_URL, so it deploys as pure static assets (Cloudflare
// Pages, any CDN) with no Node server. `images.unoptimized` is required because
// the Next image optimizer needs a server runtime that static export omits.
const nextConfig: NextConfig = {
  output: "export",
  images: { unoptimized: true }
};

export default nextConfig;
