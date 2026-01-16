// @ts-check
import { ElectronicDevice } from './lib.js';

export function isBoolean(value) {
  return typeof value === 'boolean';
}

export function isNumber(value) {
  return (
    (typeof value === 'number' && Number.isFinite(value)) ||
    typeof value === 'bigint'
  );
}

export function isObject(value) {
  return typeof value === 'object' && value !== null;
}

export function isNumericString(value) {
  return typeof value === 'string' && /^-?\d+$/.test(value);
}

export function isElectronic(object) {
  return object instanceof ElectronicDevice;
}

export function isNonEmptyArray(value) {
  return Array.isArray(value) && value.length > 0;
}

export function isEmptyArray(value) {
  return Array.isArray(value) && value.length === 0;
}

export function hasType(object) {
  return object != null && 'type' in object
}

export function assertHasId(object) {
  if (!('id' in object)) {
    throw new Error("Object is missing the 'id' property");
  }
}

export function hasIdProperty(object) {
 return object != null && object.hasOwnProperty('id')
}

export function hasDefinedType(object) {
  return Object.prototype.hasOwnProperty.call(object,'type') && object.type !== undefined
}
