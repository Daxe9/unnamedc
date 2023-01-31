<script setup lang="ts">
import { APIResponse } from "@/types/response";
import { getReasonPhrase } from "http-status-codes";

interface StatusCode {
	code: number;
	reason: string;
	description?: string;
}

const props = defineProps<{
	response: APIResponse | undefined;
}>();

function f() {
	// @ts-ignore
	const { response } = props;
	console.log(typeof response?.headers);
	console.log(response?.headers["content-length"]);
}
</script>
<template>
	<div class="response-container">
		<div class="meta-data response-child">
			<div class="specific-status status">
				<span>STATUS</span
				><span class="cool-color-text">{{
					props.response?.status_code ?? ""
				}}</span>
			</div>
			<div class="specific-status length">
				<span @click="f">LENGTH</span
				>{{
					props.response?.headers["content-length"]
						? props.response?.headers["content-length"] + " Bytes"
						: ""
				}}<span></span>
			</div>
			<div class="specific-status velocity">
				<span>VELOCITY</span
				><span class="cool-color-text">{{
					props.response?.duration ?? ""
				}}</span>
			</div>
		</div>
		<div class="response-body response-child">
			<!-- TODO: highlight the response whether is possible -->
			<component class="body-data" :is="'pre'">{{
				props.response?.body ?? ""
			}}</component>
			<div class="status-bar">
				<span>Response</span>
				<span class="response-date">{{
					props.response
						? "Last call @" + new Date().toISOString()
						: "No calls made"
				}}</span>
			</div>
		</div>
	</div>
</template>
<style>
.response-container {
	background-color: var(--response-background-color);
}

.response-child {
	margin: 0 auto;
	width: 96%;
}

.response-body {
	color: var(--response-text-color);
}

.response-date {
	text-align: right;
}

.body-data {
	background-color: var(--response-background-body-color);
	min-height: 10rem;
	overflow: scroll;
	padding: 0.5em 0.8em;
	margin-bottom: 0.5em;
	border-top-left-radius: 10px;
	border-top-right-radius: 10px;
}

.status-bar {
	background-color: transparent;
	padding: 0.2em 0.5em 0.25rem 0.5em;
	font-weight: bold;
	background-color: var(--response-background-body-color);
	border-bottom-left-radius: 10px;
	border-bottom-right-radius: 10px;
	display: flex;
	justify-content: space-between;
}

/* <div class="specific-status status"><span>STATUS</span></div> */
.specific-status span:first-child {
	font-weight: bold;
	text-align: center;
	display: inline-block;
	min-width: 6em;
	border-right: 1px solid white;
	margin-right: 0.5em;
}

.meta-data {
	background-color: black;
}
</style>
