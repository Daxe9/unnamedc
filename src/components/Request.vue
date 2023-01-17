<script setup lang="ts">
import { ref } from "vue";
import UrlBar from "./UrlBar.vue";
import { invoke } from "@tauri-apps/api";
import { APIResponse } from "@/types/response";

const emits = defineEmits(["emitResponse"])
const urlBarRef = ref();
const result = ref<APIResponse>();

function getUrlInput() {
	return urlBarRef.value.urlInput;
}

async function makeRequest() {
	const url = getUrlInput();
	result.value = await invoke("get_request", { url });
    try {
        result.value.body = JSON.parse(result.value.body);
    } catch (_) {}

    emits("emitResponse", JSON.parse(JSON.stringify(result.value)));
}
</script>
<template>
	<div class="request-content">
		<UrlBar ref="urlBarRef" @urlInput="makeRequest" />
		<div class="content">
			<div class="meta">
            </div>

			<div class="body">
			</div>
		</div>
	</div>
</template>

<style scoped>
.request-content {
	background-color: #242238;
}
</style>
