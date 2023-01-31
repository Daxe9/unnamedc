<script setup lang="ts">
import { ref, unref } from "vue";
import UrlBar from "./UrlBar.vue";
import Panel from "./Panel.vue";
import { APIResponse } from "@/types/response";
import { getRequest } from "@/services/requestService";

const emits = defineEmits(["emitResponse"]);
const urlBarRef = ref();
const result = ref<APIResponse>();

function getUrlInput() {
	return urlBarRef.value.urlInput;
}

async function makeRequest() {
	const url = getUrlInput();
	try {
        result.value = await getRequest(url);
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
.request-container {
	background-color: var(--request-background-color);
}
</style>
