import { describe, it, expect } from 'vitest';
import { parseHeaders, defaultConfig, sendWebhook } from '../src/lib/webhook.js';

describe('parseHeaders', () => {
  it('parses simple Key: Value pairs', () => {
    const h = parseHeaders('X-Auth: secret\nContent-Type: application/json');
    expect(h).toEqual({ 'X-Auth': 'secret', 'Content-Type': 'application/json' });
  });

  it('skips blank lines and # comments', () => {
    const h = parseHeaders('# this is a comment\n\nX-Real: yes\n# another comment');
    expect(h).toEqual({ 'X-Real': 'yes' });
  });

  it('handles values that contain colons', () => {
    const h = parseHeaders('Authorization: Bearer abc:def:ghi');
    expect(h.Authorization).toBe('Bearer abc:def:ghi');
  });

  it('ignores malformed lines without colon', () => {
    const h = parseHeaders('not_a_header_line\nX-OK: good');
    expect(h).toEqual({ 'X-OK': 'good' });
  });

  it('trims whitespace around key and value', () => {
    const h = parseHeaders('  X-Spaced  :   trimmed value  ');
    expect(h).toEqual({ 'X-Spaced': 'trimmed value' });
  });
});

describe('sendWebhook URL validation', () => {
  it('rejects empty URL', async () => {
    const res = await sendWebhook(defaultConfig(), {
      timestamp: Date.now(),
      port: 'test',
      protocol: 'test',
      confidence: 0,
      fields: {},
    });
    expect(res.ok).toBe(false);
    expect(res.error).toMatch(/empty/i);
  });

  it('rejects non-http(s) protocol', async () => {
    const cfg = { ...defaultConfig(), url: 'file:///etc/passwd', enabled: true };
    const res = await sendWebhook(cfg, {
      timestamp: Date.now(),
      port: 'test',
      protocol: 'test',
      confidence: 0,
      fields: {},
    });
    expect(res.ok).toBe(false);
    expect(res.error).toMatch(/http/i);
  });

  it('rejects malformed URL', async () => {
    const cfg = { ...defaultConfig(), url: 'not a url', enabled: true };
    const res = await sendWebhook(cfg, {
      timestamp: Date.now(),
      port: 'test',
      protocol: 'test',
      confidence: 0,
      fields: {},
    });
    expect(res.ok).toBe(false);
    expect(res.error).toMatch(/url/i);
  });
});
