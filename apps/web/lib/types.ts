export type Kind = "housing" | "marketplace" | "ride";
export type Listing = {
  id: string; kind: Kind; title: string; description: string; price: number; area: string;
  owner_name: string; created_at: string; status: string; image?: string; meta: Record<string, string>;
};
export type Session = { token: string; email: string; display_name: string; admin: boolean };

