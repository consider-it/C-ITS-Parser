const assert = require('node:assert/strict');
const { readFileSync } = require('node:fs');
const parser = require('../pkg/c_its_parser.js');

const cam = readFileSync(`${__dirname}/files/cam.uper`);
const decoded = parser.decode(cam, parser.Headers.None, parser.EncodingRules.JER);
assert.equal(decoded.message_type, 2);
assert.equal(JSON.parse(decoded.its).header.stationID, 0xde140ce5);
const message = new parser.JsonItsMessage(decoded.its, undefined, undefined, 2);
assert.deepEqual(Buffer.from(parser.encodeCam(message, 141)), cam);
message.free();
decoded.free();
