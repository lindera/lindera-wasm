import { TokenizerBuilder, getVersion } from '../../pkg/lindera_wasm.js';

// Show the version in the title
try {
    const version = getVersion();
    document.title = `Lindera WASM v${version}`;
    document.getElementById('title').textContent = `Lindera WASM v${version}`;
} catch (e) {
    console.error("Failed to get version:", e);
}

// Initialize the tokenizer
let tokenizer = null;

try {
    // Create a TokenizerBuilder instance
    let builder = new TokenizerBuilder();
    // Set the dictionary to "ipadic" (Japanese)
    // You can also use "ko-dic" (Korean) or "cc-cedict" (Chinese) as the dictionary
    builder.setDictionary("embedded://ipadic");

    // Set the tokenizer mode to "normal"
    // You can also use "decompose" for decomposing the compound words into their components
    builder.setMode("normal");

    // Append character filters
    builder.appendCharacterFilter("unicode_normalize", { "kind": "nfkc" });

    // Append token filters
    builder.appendTokenFilter("lowercase");
    builder.appendTokenFilter("japanese_compound_word", {
        "kind": "ipadic",
        "tags": [
            "名詞,数"
        ],
        "new_tag": "名詞,数"
    });
    builder.appendTokenFilter("japanese_number", { "tags": ["名詞,数"] });

    // Build the Tokenizer instance
    tokenizer = builder.build();

    console.log("Tokenizer is ready.");
} catch (e) {
    // Handle the error
    console.error("Failed to create Tokenizer:", e);
}

// Add an event listener to the "runButton" element
document.getElementById('runButton').addEventListener('click', () => {
    // If the tokenizer is not initialized yet, display an error message
    if (!tokenizer) {
        console.error("Tokenizer is not initialized yet.");
        return;
    }

    // Get the input text from the "inputText" element
    const inputText = document.getElementById('inputText').value;

    // Tokenize the input text
    const tokens = tokenizer.tokenize(inputText);

    // Get the "resultList" element
    const resultList = document.getElementById('resultList');

    // Clear the previous results
    resultList.innerHTML = '';

    // Display the tokens
    tokens.forEach(token => {
        const li = document.createElement('li');
        li.textContent = token.get('text') + ' : ' + token.get('details').join(", "); // Display the text of the token
        resultList.appendChild(li);
    });
});
