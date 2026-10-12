-- PIXORA access-key issuance. Apply only to a dedicated PIXORA Supabase project.
create extension if not exists pgcrypto;

create table if not exists public.pixora_access_keys (
  id uuid primary key default gen_random_uuid(),
  key_hash text not null unique,
  status text not null default 'issued' check (status in ('issued', 'active', 'expired', 'revoked')),
  issued_at timestamptz not null default now(),
  activation_deadline timestamptz not null default (now() + interval '24 hours'),
  activated_at timestamptz,
  expires_at timestamptz,
  installation_public_key text,
  created_at timestamptz not null default now(),
  check (
    (status = 'issued' and activated_at is null and expires_at is null and installation_public_key is null)
    or (status = 'active' and activated_at is not null and expires_at is not null and installation_public_key is not null)
    or status in ('expired', 'revoked')
  )
);

create index if not exists pixora_access_keys_status_expiry_idx
  on public.pixora_access_keys (status, expires_at);

alter table public.pixora_access_keys enable row level security;
-- No client-side policies: only Edge Functions using the service role may access this table.
