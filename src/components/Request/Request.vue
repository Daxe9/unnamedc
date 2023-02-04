<script setup lang="ts">
import { ref } from "vue";
import UrlBar from "./UrlBar.vue";
import Panel from "./Panel.vue";
import ErrorPanel from "@/components/Error/ErrorPanel.vue";
import type { APIRequest } from "@/types/request";

const emits = defineEmits(["sendRequest"]);
const props = defineProps<{
    renderError: string;
}>();

const urlBarRef = ref();
const settingsRef = ref();

function getUrlInput() {
	return urlBarRef.value.urlInput;
}

function buildRequest() {
    // TODO: headers, body
    const url = getUrlInput();
    // unwrap Proxy
    let settings = Object.assign({}, settingsRef.value.state);

    const request: APIRequest = {
        url,
        settings
    }; 

	emits("sendRequest", request);
}
</script>
<template>
	<div class="request-container">
		<UrlBar ref="urlBarRef" @urlInput="buildRequest" />
        <ErrorPanel class="error-panel" v-if="!!props.renderError" :message="props.renderError" />
		<Panel ref="settingsRef" />
		<div class="content">
			<div class="meta"></div>


			<div class="body"></div>
		</div>
	</div>
</template>

<style>
.request-container {
	background-color: var(--background-color);
}

.error-panel {
    margin-top: 0.5rem;
    margin-left: 0.4em;
    margin-right: 0.4em;
}
</style>
