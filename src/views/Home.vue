<script setup lang="ts">
import Request from "@/components/Request.vue";
import Response from "@/components/Response.vue";
import { ref } from "vue";
import { APIResponse } from "@/types/response";

const response = ref<APIResponse>();

function receiveResponse(res: APIResponse) {
    response.value = res
}

</script>
<template>
	<div class="main-container">
		<Request @emitResponse="receiveResponse"/>
		<Response :response="response" />
	</div>
</template>

<style scoped>
.main-container {
	display: flex;
	flex-direction: column;
	width: 100%;
	height: 100%;
	color: #ffffff;
}

.main-container > * {
    flex-basis: 50%;
}
</style>
