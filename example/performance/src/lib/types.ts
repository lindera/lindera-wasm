export type Token = {
	// https://github.com/takuyaa/kuromoji.js/blob/master/demo/tokenize.html#L94C2-L105C30
	surface_form: string;
	pos: string;
	pos_detail_1: string;
	pos_detail_2: string;
	pos_detail_3: string;
	conjugated_type: string;
	conjugated_form: string;
	basic_form: string;
	reading: string;
	pronunciation: string;
};
export type Tokenize = (text: string) => Promise<{
	tokens: Token[];
	loadTime: number;
	executionTime: number;
}>;
