import { Listing, Session } from "./types";
const BASE = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8081";

async function request<T>(path: string, init?: RequestInit, token?: string): Promise<T> {
  const response = await fetch(`${BASE}${path}`, { ...init, headers: { "Content-Type": "application/json", ...(token ? { Authorization: `Bearer ${token}` } : {}), ...init?.headers } });
  if (!response.ok) throw new Error((await response.json().catch(() => null))?.error || "Request failed");
  return response.json();
}
export const api = {
  listings: (kind?: string, search?: string) => request<Listing[]>(`/listings?kind=${kind || ""}&search=${encodeURIComponent(search || "")}`),
  requestOtp: (email: string) => request<{ development_code?: string }>("/auth/request-otp", { method: "POST", body: JSON.stringify({ email }) }),
  verifyOtp: (email: string, code: string, display_name: string) => request<Session>("/auth/verify-otp", { method: "POST", body: JSON.stringify({ email, code, display_name }) }),
  createListing: (data: unknown, token: string) => request<Listing>("/listings", { method: "POST", body: JSON.stringify(data) }, token),
  requestContact: (id: string, token: string) => request<{ message: string }>(`/listings/${id}/contact-request`, { method: "POST" }, token),
  report: (id: string, reason: string, token: string) => request<{ message: string }>("/reports", { method: "POST", body: JSON.stringify({ listing_id: id, reason }) }, token),
  seed: () => request<{ inserted: number }>("/dev/seed", { method: "POST" })
};

