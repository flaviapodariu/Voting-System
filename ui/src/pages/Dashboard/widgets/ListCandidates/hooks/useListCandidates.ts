import { useEffect, useState } from 'react';

import { useGetNetworkConfig } from 'hooks';
import { ContractFunction, ResultsParser, ProxyNetworkProvider } from 'utils';
import { smartContract } from 'utils/smartContract';

const resultsParser = new ResultsParser();

export const useListCandidates = () => {
    const { network } = useGetNetworkConfig();
    const [candidates, setCandidates] = useState<any>();

    const proxy = new ProxyNetworkProvider(network.apiAddress);

    const getCandidates = async () => {
        try {
            const query = smartContract.createQuery({
                func: new ContractFunction('getCandidates')
            });
            const queryResponse = await proxy.queryContract(query);

            const endpointDefinition = smartContract.getEndpoint('getCandidates');

            const result = resultsParser.parseQueryResponse(
                queryResponse,
                endpointDefinition
            );


            const data: any = (result?.firstValue as any)?.["items"];

            const finalData = data?.map((d: any) => {
                return String.fromCharCode.apply(null, d.value);
            })

            setCandidates(finalData);

        } catch (err) {
            console.error('Unable to call getCandidates', err);
        }
    };

    useEffect(() => {
        getCandidates();
    }, []);

    return { candidates: candidates };
};
