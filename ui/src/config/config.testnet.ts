import { EnvironmentsEnum } from 'types';

export * from './sharedConfig';

export const contractAddress =
  'erd1qqqqqqqqqqqqqpgqpw4hct7st4m5lezgyjluczjd6jzpaj0p3r5q4xy3y5';
export const API_URL = 'https://testnet-template-api.multiversx.com';
export const sampleAuthenticatedDomains = [API_URL];
export const environment = EnvironmentsEnum.testnet;
