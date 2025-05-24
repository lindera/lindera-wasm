<script lang="ts">
	import CONSTANTS from '$lib/constants';
	import { tokenizeByJavaScript, tokenizeByWASM } from '$lib';
	import Spinner from '$lib/components/Spinner.svelte';
	import TokenResult from '$lib/components/TokenResult.svelte';

	const dataSources = $state(CONSTANTS.texts);

	let selected = $state(0);
	let state = $state({
		isRunning: false,
		JavaScript: {
			tokens: [],
			loadTime: 0,
			executionTime: 0
		},
		WASM: {
			tokens: [],
			loadTime: 0,
			executionTime: 0
		}
	});
	function processTime(s) {
		return s.executionTime - s.loadTime;
	}

	async function runComparison() {
		// initialize
		state.JavaScript.tokens = [];
		state.WASM.tokens = [];
		state.isRunning = true;

		const resultJS = await tokenizeByJavaScript(CONSTANTS.texts[selected].text);
		state.JavaScript.tokens = resultJS.tokens;
		state.JavaScript.loadTime = resultJS.loadTime;
		state.JavaScript.executionTime = resultJS.executionTime;
		const resultWASM = await tokenizeByWASM(CONSTANTS.texts[selected].text);
		state.WASM.tokens = resultWASM.tokens;
		state.WASM.loadTime = resultWASM.loadTime;
		state.WASM.executionTime = resultWASM.executionTime;
	}
</script>

<div class="container">
	<h1>パフォーマンス比較</h1>
	<div class="data-source-selector">
		<label for="data-source">データソース:</label>
		<select id="data-source" bind:value={selected}>
			{#each dataSources as dataSource}
				<option value={dataSource.id}>
					{dataSource.display}
				</option>
			{/each}
		</select>
	</div>
	<div class="controls">
		<button onclick={runComparison}>比較開始</button>
	</div>
	{#if state.isRunning}
		<div id="comparison-results" class="comparison-container">
			{#if state.JavaScript.tokens.length === 0}
				<Spinner />
			{:else}
				<div class="comparison-item">
					<div class="implementation-name">JavaScript実装</div>
					<div class="execution-time">
						総実行時間: <span
							class="result-time {state.JavaScript.executionTime < state.WASM.executionTime
								? 'fastest'
								: 'slowest'}"
						>
							{state.JavaScript.executionTime.toFixed(2) + ' ms'}
						</span>
					</div>
					<div class="execution-time">
						辞書読み込み時間: <span
							class="result-time {state.JavaScript.loadTime < state.WASM.loadTime
								? 'fastest'
								: 'slowest'}"
						>
							{state.JavaScript.loadTime.toFixed(2) + ' ms'}
						</span>
					</div>
					<div class="execution-time">
						トークナイズ実行時間: <span
							class="result-time {processTime(state.JavaScript) < processTime(state.WASM)
								? 'fastest'
								: 'slowest'}"
						>
							{processTime(state.JavaScript).toFixed(2) + ' ms'}
						</span>
					</div>
					<div class="result-container">
						<span class="result-label">出力:</span>
						<TokenResult tokens={state.JavaScript.tokens}/>
					</div>
				</div>
			{/if}
			{#if state.WASM.tokens.length === 0}
				<Spinner />
			{:else}
				<div class="comparison-item">
					<div class="implementation-name">WASM実装</div>
					<div class="execution-time">
						総実行時間: <span
							class="result-time {state.WASM.executionTime < state.JavaScript.executionTime
								? 'fastest'
								: 'slowest'}"
						>
							{state.WASM.executionTime.toFixed(2) + ' ms'}
						</span>
					</div>
					<div class="execution-time">
						辞書読み込み時間: <span
							class="result-time {state.WASM.loadTime < state.JavaScript.loadTime
								? 'fastest'
								: 'slowest'}"
						>
							{state.WASM.loadTime.toFixed(2) + ' ms'}
						</span>
					</div>
					<div class="execution-time">
						トークナイズ実行時間: <span
							class="result-time {processTime(state.WASM) < processTime(state.JavaScript)
								? 'fastest'
								: 'slowest'}"
						>
							{processTime(state.WASM).toFixed(2) + ' ms'}
						</span>
					</div>
					<div class="result-container">
						<span class="result-label">出力:</span>
						<TokenResult tokens={state.WASM.tokens}/>
					</div>
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.container {
		background-color: #fff;
		padding: 30px;
		border-radius: 8px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
		text-align: center;
	}

	.data-source-selector {
		margin-bottom: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.data-source-selector label {
		margin-right: 10px;
		font-weight: bold;
		color: #555;
	}

	.data-source-selector select {
		padding: 8px 12px;
		border: 1px solid #ccc;
		border-radius: 4px;
		font-size: 16px;
	}

	.comparison-container {
		display: flex;
		gap: 20px; /* 項目間のスペース */
		margin-bottom: 20px;
	}

	.comparison-item {
		flex: 1; /* 各項目が同じ幅になるように */
		border: 1px solid #ddd;
		padding: 15px;
		border-radius: 6px;
		text-align: left; /* 内部のテキストは左寄せ */
		position: relative; /* スピナーの基準位置 */
	}

	.implementation-name {
		font-weight: bold;
		color: #555;
		margin-bottom: 10px;
	}

	.controls button {
		padding: 10px 20px;
		margin:8px;
		background-color: #28a745;
		color: white;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		font-size: 16px;
		transition: background-color 0.3s ease;
		margin-right: 10px;
	}

	.controls button:hover {
		background-color: #218838;
	}

	.result-container {
		margin-top: 10px;
	}

	.result-label {
		font-weight: bold;
		color: #777;
		margin-right: 5px;
	}

	.execution-time {
		font-size: 18px;
		margin-top: 5px;
	}

	.fastest {
		color: green;
		font-weight: bold;
	}

	.slowest {
		color: red;
		font-weight: bold;
	}
</style>
