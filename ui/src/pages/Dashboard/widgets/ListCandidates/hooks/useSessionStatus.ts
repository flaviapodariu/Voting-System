import { useEffect, useState } from 'react';

import { useGetNetworkConfig } from 'hooks';
import { ContractFunction, ResultsParser, ProxyNetworkProvider } from 'utils';
import { smartContract } from 'utils/smartContract';

const resultsParser = new ResultsParser();

export const useSessionStatus = () => {
    const { network } = useGetNetworkConfig();

    const [isActive, setIsActive] = useState<boolean | undefined>(undefined);

    const proxy = new ProxyNetworkProvider(network.apiAddress);

    const getSessionStatus = async () => {
        try {
            const query = smartContract.createQuery({
                func: new ContractFunction('is_active')
            });
            const queryResponse = await proxy.queryContract(query);

            const endpointDefinition = smartContract.getEndpoint('is_active');

            const result = resultsParser.parseQueryResponse(
                queryResponse,
                endpointDefinition
            );


            console.log(result)

            const data: any = (result?.firstValue as any)?.["value"];

            setIsActive(data === true)

        } catch (err) {
            console.error('Unable to call getCandidates', err);
        }
    };

    useEffect(() => {
        getSessionStatus();
    }, []);

    return { isActive: isActive };
};
