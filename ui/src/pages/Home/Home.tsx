import { AuthRedirectWrapper, PageWrapper } from 'wrappers';
import { Transaction } from './Transaction';
import {MxLink} from "../../components";
import {RouteNamesEnum} from "../../localConstants";
import {useMatch} from "react-router-dom";

export const Home = () => {


  const isUnlockRoute = Boolean(useMatch(RouteNamesEnum.unlock));

  const ConnectButton = isUnlockRoute ? null : (
      <MxLink to={RouteNamesEnum.unlock}>Connect</MxLink>
  );

  return (
    <AuthRedirectWrapper requireAuth={false}>
      <PageWrapper>
        <div className='flex flex-col-reverse sm:flex-row items-center h-full w-full'>
          <div className='flex items-start sm:items-center h-full sm:w-1/2 sm:bg-center'>
            <div className='flex flex-col gap-2 max-w-[70sch] text-center sm:text-left text-xl font-medium md:text-2xl lg:text-3xl'>
              <div>
                <h1>Voting System Web3 App</h1>
                <p className={'text-gray-500 py-2'}>Voting system decantralized app build on the MultiversX blockchain</p>
              </div>

              <div className={'w-[250px] text-[16px] h-[36px]'}>
              {ConnectButton}
              </div>
            </div>
          </div>
          <div className='h-4/6 bg-mvx-white bg-contain bg-no-repeat w-1/2 bg-center' />
        </div>
      </PageWrapper>
    </AuthRedirectWrapper>
  );
};
