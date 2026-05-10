# P0 Override Files

Use this folder only for temporary P0 risk acceptance.

File name format:

- `p0-<issue-id>.md`

Required fields:

1. owner
2. reason
3. mitigation
4. expiry date (max 7 days)

Template:

```md
# P0 Override: <issue-id>

- owner: @raillen
- reason: <short reason>
- mitigation: <what is in place>
- expiry_utc: 2026-04-28T00:00:00Z
```
