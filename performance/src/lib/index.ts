import kuromoji from 'kuromoji';
import { TokenizerBuilder } from 'lindera-wasm';
import type { Token, Tokenize } from './types';

export const tokenizeByJavaScript: Tokenize = (text: string) => {
	const startTime = performance.now();

	return new Promise((resolve) => {
		kuromoji.builder({ dicPath: 'dict/' }).build(function (err, tokenizer) {
			const checkpoint = performance.now();
			const result = tokenizer.tokenize(text)
			const endTime = performance.now();
			const tokens=result.map((t) => ({
				surface_form: t.surface_form,
				pos: t.pos,
				pos_detail_1: t.pos_detail_1,
				pos_detail_2: t.pos_detail_2,
				pos_detail_3: t.pos_detail_3,
				conjugated_type: t.conjugated_type,
				conjugated_form: t.conjugated_form,
				basic_form: t.basic_form,
				reading: t.reading || '',
				pronunciation: t.pronunciation || ''
			})) as Token[];
			resolve({
				tokens,
				loadTime: checkpoint - startTime,
				executionTime: endTime - startTime
			});
		});
	});
};

export const tokenizeByWASM: Tokenize = async (text: string) => {
	const startTime = performance.now();

	const builder = new TokenizerBuilder();
	builder.set_dictionary_kind('ipadic');
	const tokenizer = builder.build();

	const checkpoint = performance.now();

	const result = tokenizer.tokenize(text)
	const endTime = performance.now();
	
	const tokens=result.map((t: Map<string, string | string[]>) => ({
		surface_form: t.get('text') || '',
		pos: t.get('details')?.[0] || '',
		pos_detail_1: t.get('details')?.[1] || '',
		pos_detail_2: t.get('details')?.[2] || '',
		pos_detail_3: t.get('details')?.[3] || '',
		conjugated_type: t.get('details')?.[4] || '',
		conjugated_form: t.get('details')?.[5] || '',
		basic_form: t.get('details')?.[6] || '',
		reading: t.get('details')?.[7] || '',
		pronunciation: t.get('details')?.[8] || ''
	})) as Token[];
	return {
		tokens,
		loadTime: checkpoint - startTime,
		executionTime: endTime - startTime
	};
};
