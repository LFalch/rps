//
// Created by mads on 8/27/18.
//

#ifndef RPS_PLAYER_MADS_H
#define RPS_PLAYER_MADS_H


#include "player.h"
#include <random>

class PlayerMads : public Player
{
public:
    explicit PlayerMads(const std::string& name) :
        Player(name)
    {}

    int move (const std::vector<Move>& moveHistory) {
        std::vector<float> data = calc(moveHistory);

        std::default_random_engine generator;
        std::normal_distribution<float> distribution(data[0], data[1]);

        float rand_result = distribution(generator);
        float sigma_width = 0.8;

        if (rand_result >= 0.0-sigma_width*data[1] and rand_result <= 0.0+sigma_width*data[1])
            return  PAPER;
        else if (rand_result >= 1.0-sigma_width*data[1] and rand_result <= 1.0 + sigma_width*data[1])
            return SCISSORS;
        else if (rand_result >= 2.0-sigma_width*data[1] and rand_result <= 2.0 + sigma_width*data[1])
            return ROCK;
        else
            return rand() % 3;

    }

private:
    std::vector<float> calc(const std::vector<Move>& moveHistory)
    {
        float sum(0.0);
        float sumDiff(0.0);

        for (int i = 0 ; i < moveHistory.size() ; ++i) {
            sum += moveHistory[i].m_you;
        }
        float avg = sum/((float) moveHistory.size());

        for (int j = 0; j < moveHistory.size(); ++j) {
            sumDiff += (float) std::pow(moveHistory[j].m_you-avg, 2);
        }

        float sd = std::sqrt(sumDiff/((float) moveHistory.size()));

        return {avg, sd};
    }


};

#endif //RPS_PLAYER_MADS_H
