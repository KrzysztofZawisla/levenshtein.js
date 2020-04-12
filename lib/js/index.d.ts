interface LevenshteinJS {
    /**
     * Filter function returns filted collection by Levenshtein Algorithm.
     * @input string.
     * @distance number - can't be bigger than 255 and lower than 0.
     * @collection string[] - collection to search over.
     * @returns string[] - filtered collection.
    */
    filter: (input: string, distance: number, collection: string[]) => string[];
}
declare const lib: LevenshteinJS;
export default lib;
