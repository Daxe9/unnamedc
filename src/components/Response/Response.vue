<script setup lang="ts">
import { APIResponse } from "@/types/response";
import { onMounted } from "vue";
/* import { getReasonPhrase } from "http-status-codes"; */
import hljsVuePlugin from "@highlightjs/vue-plugin";

onMounted(() => {
    // the style is done only when the component is mounted, it can't be styled otherwise
	// @ts-ignore
	document.querySelector(".hljs").style.backgroundColor = "#191c1b";
});
const testtext = "console.log('Hello World!');";
const Highlight = hljsVuePlugin.component;
const props = defineProps<{
	response: APIResponse | undefined;
}>();
</script>
<template>
	<div class="response-container">
		<div
			class="border-wrapper meta-data response-child rounded-tl-lg rounded-tr-lg"
		>
			<div class="specific-status status">
				<span>STATUS</span
				><span class="cool-color-text">{{
					props.response?.status_code ?? ""
				}}</span>
			</div>
			<div class="specific-status length">
				<span>LENGTH</span
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
			<Highlight
                :code="props.response?.body ? JSON.stringify(props.response.body, null, '\t') : testtext " 
				class="border-wrapper body-data my-2"
                autodetect
                />

			<div class="border-wrapper status-bar">
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
	background-color: var(--background-color);
}

.response-container > div {
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
	background-color: var(--dark-color);
	min-height: 5rem;
	overflow: scroll;
	padding: 0.5em 0.8em;
}

.status-bar {
	background-color: transparent;
	padding: 0.2em 0.5em 0.25rem 0.5em;
	font-weight: bold;
	background-color: var(--dark-color);
	border-bottom-left-radius: 10px;
	border-bottom-right-radius: 10px;
	display: flex;
	justify-content: space-between;
}

.specific-status span:first-child {
	font-weight: bold;
	text-align: center;
	display: inline-block;
	min-width: 6em;
	border-right: 1px solid white;
	margin-right: 0.5em;
}

.meta-data {
	background-color: var(--dark-color);
}
</style>
