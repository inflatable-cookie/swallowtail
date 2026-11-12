import { register } from 'node:module';
const encoded = new URL(import.meta.url).hash.slice(1);
const config = JSON.parse(Buffer.from(encoded, 'base64url').toString('utf8'));
register(config.loaderUrl, { parentURL: import.meta.url, data: config });
