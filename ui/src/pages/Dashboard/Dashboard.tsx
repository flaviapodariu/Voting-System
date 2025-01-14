import { AuthRedirectWrapper } from 'wrappers';
import { useScrollToElement } from 'hooks';
import {FC, useState} from "react";
import {useListCandidates} from "./widgets/ListCandidates/hooks/useListCandidates";
import {useSessionStatus} from "./widgets/ListCandidates/hooks/useSessionStatus";
import {useGetResults} from "./widgets/ListCandidates/hooks/useGetResults";
import {useCastVoteTransaction} from "../../hooks/transactions/useCastVoteTransaction";


const VotingStatus: FC<{isActive: boolean}> = ({ isActive }) => {
  return (
      <div className="flex items-center space-x-2 py-4">
        <div
            className={`w-3 h-3 rounded-full ${isActive ? 'bg-green-500' : 'bg-red-500'}`}
        ></div>
        <span className={`text-lg font-semibold ${isActive ? 'text-green-700' : 'text-red-700'}`}>
        {isActive ? 'Voting Session is Active' : 'Voting Session is Finished'}
      </span>
      </div>
  );
};


const getButtonClassName = (c: string) => {
    if(!localStorage.getItem('userVoteData')) {
        return 'bg-green-500 text-white py-2 px-4 rounded-md text-lg hover:bg-green-600 focus:outline-none focus:ring-2 focus:ring-green-500'
    } else if(localStorage.getItem('userVoteData') !== c) {
        return 'bg-gray-500 text-white py-2 px-4 rounded-md text-lg hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-gray-500'
    } else {
        return 'bg-green-500 text-white py-2 px-4 rounded-md text-lg hover:bg-green-600 focus:outline-none focus:ring-2 focus:ring-green-500'
    }
}

export const Dashboard = () => {
  useScrollToElement();
  const {candidates} = useListCandidates();

  const {sendCastVoteTransactionFromAbi} = useCastVoteTransaction()

  const { isActive } = useSessionStatus();

  const {results} = useGetResults(isActive);

  const [voted, setVoted] = useState(localStorage.getItem('userVoteData') !== null);

  if(isActive === false) {
      localStorage.removeItem('userVoteData')
  }


  if(isActive === undefined) {
      return <div>Loading...</div>
  }
  return (
    <AuthRedirectWrapper>

      <div className='flex flex-col gap-6 max-w-5xl w-full'>

        <VotingStatus isActive={isActive} />

          {isActive ? (
              <>
                  <div className={'text-xl'}>Candidates List</div>
                  <div className="flex flex-wrap gap-6 justify-center p-6">
                      {(candidates || []).map((c: string, i: number) => (
                          <div
                              key={i}
                              className="bg-gray-100 rounded-lg w-64 p-6 shadow-lg flex flex-col items-center text-center transition-transform transform hover:translate-y-1 hover:shadow-2xl"
                          >
                              <span className="text-xl font-semibold text-gray-800">{c}</span>
                              <div className="mt-4">
                                  <button
                                      disabled={voted}
                                      className={getButtonClassName(c)}
                                      onClick={() => {

                                          sendCastVoteTransactionFromAbi({
                                              amount: '0',
                                              candidate: c,
                                              callbackRoute: '/dashboard'
                                          })

                                          localStorage.setItem('userVoteData', c)
                                          setVoted(true)
                                      }}
                                  >
                                      {c === localStorage.getItem('userVoteData') ? 'Voted' : 'Vote'}
                                  </button>
                              </div>
                          </div>

                      ))}
                  </div>
                  <div>
                      {voted && (
                          <p className={'text-center'}>Your vote has been recorded, thank you for voting!
                              Please wait for the voting session to end to see the results.</p>
                      )}

                  </div>
              </>
          ): (<div>

              <div className="overflow-x-auto py-6">
                  <table className="min-w-full bg-white border border-gray-200 shadow-md rounded-lg">
                      <thead>
                      <tr className="text-left bg-gray-100">
                          <th className="px-4 py-2 text-sm font-medium text-gray-700">Rank</th>
                          <th className="px-4 py-2 text-sm font-medium text-gray-700">Candidate</th>
                          <th className="px-4 py-2 text-sm font-medium text-gray-700">Votes</th>
                      </tr>
                      </thead>
                      <tbody>
                      {(results as {name: string, votes: number}[] || [])?.sort((a, b) => b.votes - a.votes)?.map((candidate, index) => (
                          <tr
                              key={candidate.name}
                              className={`
                ${index === 0 ? 'bg-green-300' : index % 2 === 0 ? 'bg-gray-50' : 'bg-white'} 
                hover:bg-gray-100 h-[50px] transition-colors
              `}
                          >
                              <td className="px-4 py-2 text-sm text-gray-800">{index + 1}</td>
                              <td className="px-4 py-2 text-sm text-gray-800">{candidate.name}</td>
                              <td className="px-4 py-2 text-sm text-gray-800">{candidate.votes}</td>
                          </tr>
                      ))}
                      </tbody>
                  </table>
              </div>

          </div>)}

      </div>
    </AuthRedirectWrapper>
  );
};
