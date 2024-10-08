/* tslint:disable */
/* eslint-disable */
/**
 * BlindChannel REST API
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { mapValues } from '../runtime';
/**
 * 
 * @export
 * @interface PublishMessage
 */
export interface PublishMessage {
    /**
     * 
     * @type {string}
     * @memberof PublishMessage
     */
    content: string;
    /**
     * 
     * @type {string}
     * @memberof PublishMessage
     */
    recipient: string;
}

/**
 * Check if a given object implements the PublishMessage interface.
 */
export function instanceOfPublishMessage(value: object): value is PublishMessage {
    if (!('content' in value) || value['content'] === undefined) return false;
    if (!('recipient' in value) || value['recipient'] === undefined) return false;
    return true;
}

export function PublishMessageFromJSON(json: any): PublishMessage {
    return PublishMessageFromJSONTyped(json, false);
}

export function PublishMessageFromJSONTyped(json: any, ignoreDiscriminator: boolean): PublishMessage {
    if (json == null) {
        return json;
    }
    return {
        
        'content': json['content'],
        'recipient': json['recipient'],
    };
}

export function PublishMessageToJSON(value?: PublishMessage | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'content': value['content'],
        'recipient': value['recipient'],
    };
}

