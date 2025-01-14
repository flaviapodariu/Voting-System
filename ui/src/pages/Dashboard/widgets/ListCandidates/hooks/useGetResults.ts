import { useEffect, useState } from 'react';

import { useGetNetworkConfig } from 'hooks';
import { ContractFunction, ResultsParser, ProxyNetworkProvider } from 'utils';
import { smartContract } from 'utils/smartContract';

const resultsParser = new ResultsParser();

export const useGetResults = (sessionActive: boolean = false) => {
    const { network } = useGetNetworkConfig();
    const [results, setResults] = useState<any>();

    const proxy = new ProxyNetworkProvider(network.apiAddress);

    const getResults = async (sessionActive: boolean) => {
        if(sessionActive) return [];
        try {
            const query = smartContract.createQuery({
                func: new ContractFunction('getResults')
            });
            const queryResponse = await proxy.queryContract(query);

            const endpointDefinition = smartContract.getEndpoint('getResults');

            const result = resultsParser.parseQueryResponse(
                queryResponse,
                endpointDefinition
            );

            const data: any = (result?.firstValue as any)?.["items"];

            console.log(data);

            const finalData = (data || [])?.map((d: any) => {
                return {
                    name: String.fromCharCode.apply(null, d.fields?.[0]?.value?.value),
                    votes: parseInt(d.fields?.[1]?.value?.value?.toString())
                }
            })

            console.log(finalData)

            setResults(finalData);

        } catch (err) {
            console.error('Unable to call getResults', err);
        }
    };

    useEffect(() => {
        getResults(sessionActive);
    }, [sessionActive]);

    return { results: results };
};
