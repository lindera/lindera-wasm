<script>
	let { tokens = [] } = $props();
	let active = $state(false);

	function onclick() {
		active = !active;
	}
</script>

<div class="accordion-container">
	<button type="button" class="accordion-button {active ? 'active' : ''}" {onclick}
		>形態素解析結果を表示</button
	>
	<div class="accordion-panel {active ? 'active' : ''}">
		<table>
			<thead>
				<tr>
					<th>表層形</th>
					<th>品詞</th>
					<th>品詞細分類1</th>
					<th>品詞細分類2</th>
					<th>品詞細分類3</th>
					<th>活用型</th>
					<th>活用形</th>
					<th>基本形</th>
					<th>読み</th>
					<th>発音</th>
				</tr>
			</thead>
			<tbody>
				{#each tokens as token}
					<tr>
						<td>{token.surface_form}</td>
						<td>{token.pos}</td>
						<td>{token.pos_detail_1}</td>
						<td>{token.pos_detail_2}</td>
						<td>{token.pos_detail_3}</td>
						<td>{token.conjugated_type}</td>
						<td>{token.conjugated_form}</td>
						<td>{token.reading}</td>
						<td>{token.pronunciation}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</div>

<style>
	.accordion-container {
		margin-bottom: 15px;
	}

	.accordion-button {
		background-color: #eee;
		color: #444;
		cursor: pointer;
		padding: 18px;
		width: 100%;
		border: none;
		text-align: left;
		outline: none;
		font-size: 16px;
		transition: 0.4s;
	}

	.active,
	.accordion-button:hover {
		background-color: #ccc;
	}

	.accordion-button:after {
		content: '\02795'; /* Plus sign as a Unicode character */
		font-size: 13px;
		color: #777;
		float: right;
		margin-left: 5px;
	}

	.accordion-button.active:after {
		content: '\02796'; /* Minus sign as a Unicode character */
	}

	.accordion-panel {
		max-height: 0;
		opacity: 0;
		background-color: white;
		transition:
			max-height 0.2s,
			padding 0.2s,
			opacity 0.2s;
		overflow: hidden;
	}

	.accordion-panel.active {
		max-height: 100%;
		opacity: 1;
	}

	table {
		border-collapse: collapse;
		width: 100%;
	}

	th,
	td {
		border: 1px solid #ddd;
		padding: 8px;
		text-align: left;
	}

	th {
		background-color: #f2f2f2;
		font-weight: bold;
	}
</style>
