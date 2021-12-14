//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
// 
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see http://www.gnu.org/licenses/.
// 

#ifndef __CANONICAL_TREE_VSWITCH_H_
#define __CANONICAL_TREE_VSWITCH_H_

#include <omnetpp.h>
#include "ISwitch.h"

using namespace omnetpp;

namespace simulation {

class VSwitch : public ISwitch
{
  protected:
    void initSwitch();
    void handleMessage(cMessage *msg);
    void registerReceivedSignal();
    void registerDroppedSignal();
    void sendUp(cMessage *msg);
    bool isIdle();
    void end();

  private:
    bool* vnfs_idle;
};

} //namespace

#endif
