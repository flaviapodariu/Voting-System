import { useState, useCallback } from 'react';
import { signAndSendTransactions } from 'helpers/signAndSendTransactions';
import {
  useGetAccountInfo,
  useGetNetworkConfig,
  useTrackTransactionStatus
} from 'hooks/sdkDappHooks';
import { GAS_PRICE, SessionEnum, VERSION } from 'localConstants';
import { getChainId } from 'utils/getChainId';
import { smartContract } from 'utils/smartContract';
import {
  PingRawProps,
} from 'types/pingPong.types';
import { Address } from 'utils/sdkDappCore';
import {BytesValue} from "@multiversx/sdk-core/out";

type PingPongTransactionProps = {
  type: SessionEnum;
};

const PING_TRANSACTION_INFO = {
  processingMessage: 'Processing Cast Vote transaction',
  errorMessage: 'An error has occured during Cast Vote',
  successMessage: 'Cast Vote transaction successful'
};

export const useCastVoteTransaction = () => {
  const [pingPongSessionId, setCastVoteSessionId] = useState();

  const { network } = useGetNetworkConfig();
  const { address, account } = useGetAccountInfo();

  const transactionStatus = useTrackTransactionStatus({
    transactionId: pingPongSessionId ?? '0'
  });


  const sendCastVoteTransactionFromAbi = useCallback(
    async ({ amount, candidate, callbackRoute }: any) => {

      const castVoteTransaction = smartContract.methodsExplicit
        .castVote([
            new BytesValue(Buffer.from(candidate, 'utf-8'))
        ])
        .withSender(new Address(address))
        .withValue(amount ?? '0')
        .withGasLimit(60000000)
        .withChainID(getChainId())
        .buildTransaction();

      const sessionId = await signAndSendTransactions({
        transactions: [castVoteTransaction],
        callbackRoute,
        transactionsDisplayInfo: PING_TRANSACTION_INFO
      });

      console.log(sessionId)

      // sessionStorage.setItem(type, sessionId);
      setCastVoteSessionId(sessionId);
    },
    []
  );
  //
  // const sendCastVoteTransaction = useCallback(
  //   async ({ callbackRoute }: PongRawProps) => {
  //
  //     const pongTransaction = newTransaction({
  //       value: '0',
  //       data: 'pong',
  //       receiver: contractAddress,
  //       gasLimit: 60000000,
  //       gasPrice: GAS_PRICE,
  //       chainID: network.chainId,
  //       nonce: account.nonce,
  //       sender: address,
  //       version: VERSION
  //     });
  //
  //     const sessionId = await signAndSendTransactions({
  //       transactions: [pongTransaction],
  //       callbackRoute,
  //       transactionsDisplayInfo: PONG_TRANSACTION_INFO
  //     });
  //
  //     sessionStorage.setItem(type, sessionId);
  //     setCastVoteSessionId(sessionId);
  //   },
  //   []
  // );

  return {
    sendCastVoteTransactionFromAbi,
    transactionStatus
  };
};
