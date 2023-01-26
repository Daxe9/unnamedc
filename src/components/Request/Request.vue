<script setup lang="ts">
import { ref, unref } from "vue";
import UrlBar from "./UrlBar.vue";
import Panel from "./Panel.vue";
import { invoke } from "@tauri-apps/api";
import { APIResponse } from "@/types/response";

const emits = defineEmits(["emitResponse"]);
const urlBarRef = ref();
const result = ref<APIResponse>();

function getUrlInput() {
	return urlBarRef.value.urlInput;
}

async function makeRequest() {
	const url = getUrlInput();
	result.value = await invoke("get_request", { url });
	try {
		// @ts-ignore
		result.value.body = JSON.parse(result.value.body);
	} catch (_) {}

	emits("emitResponse", unref(result.value));
}
</script>
<template>
	<div class="request-container">
		<UrlBar ref="urlBarRef" @urlInput="makeRequest" />
        <Panel />
		<div class="content">
			<div class="meta"></div>

			<div class="body"></div>
		</div>
	</div>
</template>

<style>
@import "@/main.css";
.request-container {
	background-color: var(--request-background-color);
}
</style>
